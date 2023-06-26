// ---------------------------------------------------------------------
// UDP utilities
// ---------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// See LICENSE for details
// ---------------------------------------------------------------------

use bytes::BytesMut;
use common::{AgentError, AgentResult};
use frame::{FrameReader, FrameWriter};
use std::net::SocketAddr;
use tokio::net::{ToSocketAddrs, UdpSocket};

#[derive(Debug)]
pub struct UdpConnection {
    socket: UdpSocket,
    buffer: BytesMut,
}

const UDP_BUFF_CAPACITY: usize = 16384;

impl UdpConnection {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<UdpConnection> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(UdpConnection {
            socket,
            buffer: BytesMut::with_capacity(UDP_BUFF_CAPACITY),
        })
    }
    pub fn local_port(&self) -> Result<u16, AgentError> {
        Ok(self.socket.local_addr()?.port())
    }
    pub fn set_ttl(&self, ttl: u32) -> Result<(), AgentError> {
        self.socket.set_ttl(ttl)?;
        Ok(())
    }
    // Connect socket to first packet's sender
    pub async fn connect_to_sender(&mut self) -> AgentResult<SocketAddr> {
        let addr = self
            .socket
            .peek_sender()
            .await
            .map_err(|e| AgentError::NetworkError(e.to_string()))?;
        self.socket
            .connect(addr)
            .await
            .map_err(|e| AgentError::NetworkError(e.to_string()))?;
        Ok(addr)
    }
    pub async fn recv<T: FrameReader>(&mut self) -> AgentResult<T> {
        self.buffer.clear();
        loop {
            self.socket.readable().await?;
            match self.socket.try_recv_buf(&mut self.buffer) {
                Ok(_) => return T::parse(&mut self.buffer),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(AgentError::NetworkError(e.to_string()));
                }
            }
        }
    }
    pub async fn recv_from<T: FrameReader>(&mut self) -> Result<(T, SocketAddr), AgentError> {
        self.buffer.clear();
        loop {
            self.socket.readable().await?;
            match self.socket.try_recv_buf_from(&mut self.buffer) {
                Ok((_, addr)) => {
                    let r = T::parse(&mut self.buffer)?;
                    return Ok((r, addr));
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(AgentError::NetworkError(e.to_string()));
                }
            }
        }
    }
    pub async fn send_to<T: FrameWriter>(
        &mut self,
        frame: &T,
        addr: SocketAddr,
    ) -> Result<usize, AgentError> {
        self.buffer.clear();
        frame.write_bytes(&mut self.buffer)?;
        Ok(self.socket.send_to(&self.buffer, addr).await?)
    }
    // Send to the previously connected socket
    pub async fn send<T: FrameWriter>(&mut self, frame: &T) -> AgentResult<usize> {
        self.buffer.clear();
        frame.write_bytes(&mut self.buffer)?;
        Ok(self.socket.send(&self.buffer).await?)
    }
}

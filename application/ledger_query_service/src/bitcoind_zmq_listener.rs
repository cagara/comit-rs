use bitcoin_support::{serialize::deserialize, BlockWithHeight};
use block_processor::BlockProcessor;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use zmq::{self, Context, Socket};

#[derive(DebugStub)]
pub struct BitcoindZmqListener<P> {
    #[debug_stub = "Context"]
    _context: Context,
    #[debug_stub = "Socket"]
    socket: Socket,
    #[debug_stub = "Processor"]
    processor: P,
}

impl<P: BlockProcessor<BlockWithHeight>> BitcoindZmqListener<P> {
    pub fn new(endpoint: &str, processor: P) -> Result<Self, zmq::Error> {
        let context = Context::new()?;
        let mut socket = context.socket(zmq::SUB)?;

        socket.set_subscribe(b"rawblock")?;
        socket.connect(endpoint)?;

        Ok(BitcoindZmqListener {
            _context: context,
            socket,
            processor,
        })
    }

    pub fn start(&mut self) {
        info!(
            "Connecting to {} to subscribe to new Bitcoin blocks over ZeroMQ",
            self.socket.get_last_endpoint().unwrap()
        );

        loop {
            let result = self.receive_block();

            if let Ok(Some(block)) = result {
                self.processor.process(&block);
            }
        }
    }

    fn receive_block(&mut self) -> Result<Option<BlockWithHeight>, zmq::Error> {
        let bytes = self.socket.recv_bytes(zmq::SNDMORE)?;
        let bytes: &[u8] = bytes.as_ref();

        match bytes {
            b"rawblock" => {
                let bytes = self.socket.recv_bytes(zmq::SNDMORE)?;
                let end_bytes = self.socket.recv_bytes(zmq::SNDMORE)?;

                let mut end_bytes = Cursor::new(end_bytes);
                let height = end_bytes.read_u32::<LittleEndian>();

                match deserialize(bytes.as_ref()) {
                    Ok(block) => {
                        trace!("Got {:?}", block);
                        match height {
                            Ok(height) => Ok(Some(BlockWithHeight { block, height })),
                            Err(e) => {
                                error!(
                                    "Got new block but failed to extract the height because {:?}",
                                    e
                                );
                                Ok(None)
                            }
                        }
                    }
                    Err(e) => {
                        error!("Got new block but failed to deserialize it because {:?}", e);
                        Ok(None)
                    }
                }
            }
            _ => {
                debug!("Unhandled message: {:?}", bytes);
                Ok(None)
            }
        }
    }
}

use hyper::Request;
use slog::{debug, Logger};
use tokio::sync::mpsc;

use crate::redirect::NetworkRequests;

#[derive(Clone, Debug)]
pub struct NetworkSender<N: NetworkRequests> {
    pub sender: mpsc::UnboundedSender<Request<N>>,
    pub log: Logger,
}

impl<N: NetworkRequests + std::fmt::Debug> NetworkSender<N> {
    pub fn new(log: Logger) -> Self {
        let (sender, _) = mpsc::unbounded_channel();
        NetworkSender { sender, log }
    }

    pub fn send_request(
        &self,
        request: Request<N>,
    ) -> Result<(), mpsc::error::SendError<Request<N>>> {
        debug!(self.log, "Sending request to network"; "request" => format!("{:?}", request));
        self.sender.send(request)
    }
}

// /* RPC Response type - used for outbound upgrades */
// /* Outbound upgrades */
// pub type OutboundFramed<TSocket, TSpec> = Framed<Compat<TSocket>, OutboundCodec<TSpec>>;

// impl<TSocket, TSpec> OutboundUpgrade<TSocket> for OutboundRequestContainer<TSpec>
// where
//     TSpec: EthSpec + Send + 'static,
//     TSocket: AsyncRead + AsyncWrite + Unpin + Send + 'static,
// {
//     type Output = OutboundFramed<TSocket, TSpec>;
//     type Error = RPCError;
//     type Future = BoxFuture<'static, Result<Self::Output, Self::Error>>;

//     fn upgrade_outbound(self, socket: TSocket, protocol: Self::Info) -> Self::Future {
//         // convert to a tokio compatible socket
//         let socket = socket.compat();
//         let codec = match protocol.encoding {
//             Encoding::SSZSnappy => {
//                 let ssz_snappy_codec = BaseOutboundCodec::new(SSZSnappyOutboundCodec::new(
//                     protocol,
//                     self.max_rpc_size,
//                     self.fork_context.clone(),
//                 ));
//                 OutboundCodec::SSZSnappy(ssz_snappy_codec)
//             }
//         };

//         let mut socket = Framed::new(socket, codec);

//         async {
//             socket.send(self.req).await?;
//             socket.close().await?;
//             Ok(socket)
//         }
//         .boxed()
//     }
// }
use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};
use concat_string::concat_string;

use crate::client::WebsocketClient;

#[macro_use(concat_string)]

impl WebsocketClient {

    pub async fn markets(&self) -> anyhow::Result<()> {

        let (socket, _response) = connect_async(self.websocket()).await?;
        let (mut write, read) = socket.split();

        let message = r#"{ "type": "subscribe", "channel": "v4_markets" }"#;
        write.send(message.into());
        // Ok(read)
        unimplemented!();
    }

    pub async fn trades<T: AsRef<str>>(&self, ticker: T) -> anyhow::Result<()> {

        let (socket, _response) = connect_async(self.websocket()).await?;
        let (mut write, read) = socket.split();

        let message = r#"{ "type": "subscribe", "channel": "v4_trades", "id": ""#;
        let message = concat_string!(message, ticker, "\" }");

        write.send(message.into());
        // Ok(read)
        unimplemented!();
    }

    pub async fn subaccounts<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<()> {

        let (socket, _response) = connect_async(self.websocket()).await?;
        let (mut write, read) = socket.split();

        let message = r#"{ "type": "subscribe", "channel": "v4_subaccounts", "id": ""#;
        let message = concat_string!(message, address, "/", subaccount_number, "\" }");

        write.send(message.into());
        // Ok(read)
        unimplemented!();
    }


}

use crate::{Action, State};
use thirtyfour::prelude::*;

pub async fn exec(st: State) -> WebDriverResult<()> {
    match &st.act {
        Action::Test => {
            // if there is a tkl-webcp page we're good
            st.wd.goto(st.url.as_str()).await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-tklwebcp.png"))
                .await?;
            Ok(())
        }
        Action::Install => {
            // there is nothing to install for nodejs
            Ok(())
        }
    }
}

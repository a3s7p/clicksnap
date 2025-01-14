use crate::{Action, State};
use thirtyfour::prelude::*;

pub async fn exec(st: State) -> WebDriverResult<()> {
    match &st.act {
        Action::Test => {
            // landing page
            st.wd.goto(st.url.as_str()).await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-landing.png"))
                .await?;
            // db example
            let dbe = st
                .wd
                .query(By::XPath(
                    "//a[contains(@class, 'nav-link') and text() = 'DBExample']",
                ))
                .first()
                .await?;
            dbe.wait_until().displayed().await?;
            dbe.click().await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-db-example.png"))
                .await?;
            // privacy policy demo
            let pp = st
                .wd
                .query(By::XPath(
                    "//a[contains(@class, 'nav-link') and text() = 'Privacy']",
                ))
                .first()
                .await?;
            pp.wait_until().displayed().await?;
            pp.click().await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-privacy-example.png"))
                .await?;
            Ok(())
        }
        Action::Install => {
            // there is nothing to install
            Ok(())
        }
    }
}

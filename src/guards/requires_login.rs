pub struct TokenGuard;

impl guard::Guard for TokenGuard {
    fn check(&self, ctx: &guard::GuardContext) -> bool {
        let headers: &HeaderMap = ctx.head().headers();

        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(token) = auth_header.to_str() {



                return token == VALID_TOKEN;
            }
        }

        false
    }
}
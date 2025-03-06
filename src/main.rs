use cmg::{request, Request, Var};
use dropshot::{endpoint, ApiDescription, HttpError, HttpResponseOk, Query, RequestContext};

#[tokio::main]
pub async fn main() -> Result<(), String> {
    let mut api = ApiDescription::new();
    api.register(get_problem).unwrap();
     api.openapi("CMG", semver::Version::new(1, 0, 0)).write(&mut std::io::stdout()).map_err(|e| e.to_string())?;
    Ok(())

}

const TWO_DOT_OH: semver::Version = semver::Version::new(2, 0, 0);
/// Fetch `thing1`
    #[endpoint {
        method = POST,
        path = "/problem",
        versions = "1.0.0"..TWO_DOT_OH
        
    }]
    pub async fn get_problem(
        _rqctx: RequestContext<()>,
        query: Query<Request>

    ) -> Result<HttpResponseOk<Var>, HttpError> {
        let q = query.into_inner();
        q.
        

        Ok(HttpResponseOk(Var::default()))
    }

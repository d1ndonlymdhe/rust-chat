pub mod db;
pub mod routes;

#[derive(Debug,Clone, Copy)]
pub struct AnyErr(pub ());
impl From<()> for AnyErr {
    fn from(_value: ()) -> Self {
        return AnyErr(());
    }
}

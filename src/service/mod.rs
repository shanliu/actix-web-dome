pub(crate) mod account;


pub(crate) trait Model{
    fn loaded()->bool{
        return true;
    }
}
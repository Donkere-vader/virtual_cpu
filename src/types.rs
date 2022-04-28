
pub enum OppCode {
    Setr(Ubit),
    Load(Ubit),
    Store(Ubit),
    Add(Ubit),
    Sub(Ubit),
    Mul(Ubit),
    Div(Ubit),
    Jump(Ubit)
}

pub type Ubit = u8;

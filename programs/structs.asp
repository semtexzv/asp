
#derive Clone, Copy
struct Newtype(pub uint)

struct Name {
    value string
}
struct Rc of T {
    value ptr T
}
struct ErrorFmt {
    msg: String,
    code: i32,
}

enum AuthError {
    LoginErr(ErrorFmt),
    TokenExpire(ErrorFmt),
}

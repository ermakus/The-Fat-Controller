// This file was generated by build.rs

struct Flags: OptionSet {
    let rawValue: UInt8

    static let null = Flags([])
    static let capsLock = Flags(rawValue: 1)
    static let shift = Flags(rawValue: 2)
    static let control = Flags(rawValue: 4)
    static let alt = Flags(rawValue: 8)
    static let command = Flags(rawValue: 16)
}

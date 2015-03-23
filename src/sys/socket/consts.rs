pub use self::os::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod os {
    use libc::{c_int, uint8_t};

    pub const AF_UNIX: c_int  = 1;
    pub const AF_LOCAL: c_int = AF_UNIX;
    pub const AF_INET: c_int  = 2;
    pub const AF_INET6: c_int = 10;

    pub const SOCK_STREAM: c_int = 1;
    pub const SOCK_DGRAM: c_int = 2;
    pub const SOCK_SEQPACKET: c_int = 5;
    pub const SOCK_RAW: c_int = 3;
    pub const SOCK_RDM: c_int = 4;

    pub const SOL_IP: c_int     = 0;
    pub const SOL_SOCKET: c_int = 1;
    pub const SOL_TCP: c_int    = 6;
    pub const SOL_UDP: c_int    = 17;
    pub const SOL_IPV6: c_int   = 41;
    pub const IPPROTO_IP: c_int = SOL_IP;
    pub const IPPROTO_IPV6: c_int = SOL_IPV6;
    pub const IPPROTO_TCP: c_int = SOL_TCP;
    pub const IPPROTO_UDP: c_int = SOL_UDP;

    pub const SO_ACCEPTCONN: c_int = 30;
    pub const SO_BINDTODEVICE: c_int = 25;
    pub const SO_BROADCAST: c_int = 6;
    pub const SO_BSDCOMPAT: c_int = 14;
    pub const SO_DEBUG: c_int = 1;
    pub const SO_DOMAIN: c_int = 39;
    pub const SO_ERROR: c_int = 4;
    pub const SO_DONTROUTE: c_int = 5;
    pub const SO_KEEPALIVE: c_int = 9;
    pub const SO_LINGER: c_int = 13;
    pub const SO_MARK: c_int = 36;
    pub const SO_OOBINLINE: c_int = 10;
    pub const SO_PASSCRED: c_int = 16;
    pub const SO_PEEK_OFF: c_int = 42;
    pub const SO_PEERCRED: c_int = 17;
    pub const SO_PRIORITY: c_int = 12;
    pub const SO_PROTOCOL: c_int = 38;
    pub const SO_RCVBUF: c_int = 8;
    pub const SO_RCVBUFFORCE: c_int = 33;
    pub const SO_RCVLOWAT: c_int = 18;
    pub const SO_SNDLOWAT: c_int = 19;
    pub const SO_RCVTIMEO: c_int = 20;
    pub const SO_SNDTIMEO: c_int = 21;
    pub const SO_REUSEADDR: c_int = 2;
    pub const SO_REUSEPORT: c_int = 15;
    pub const SO_RXQ_OVFL: c_int = 40;
    pub const SO_SNDBUF: c_int = 7;
    pub const SO_SNDBUFFORCE: c_int = 32;
    pub const SO_TIMESTAMP: c_int = 29;
    pub const SO_TYPE: c_int = 3;
    pub const SO_BUSY_POLL: c_int = 46;

    // Socket options for TCP sockets
    pub const TCP_NODELAY: c_int = 1;
    pub const TCP_MAXSEG: c_int = 2;
    pub const TCP_CORK: c_int = 3;

    // Socket options for the IP layer of the socket
    pub const IP_MULTICAST_IF: c_int = 32;

    pub type IpMulticastTtl = uint8_t;

    pub const IP_MULTICAST_TTL: c_int = 33;
    pub const IP_MULTICAST_LOOP: c_int = 34;
    pub const IP_ADD_MEMBERSHIP: c_int = 35;
    pub const IP_DROP_MEMBERSHIP: c_int = 36;

    pub type InAddrT = u32;

    // Declarations of special addresses
    pub const INADDR_ANY: InAddrT = 0;
    pub const INADDR_NONE: InAddrT = 0xffffffff;
    pub const INADDR_BROADCAST: InAddrT = 0xffffffff;

    pub type SockMessageFlags = i32;
    // Flags for send/recv and their relatives
    pub const MSG_OOB: SockMessageFlags = 0x1;
    pub const MSG_PEEK: SockMessageFlags = 0x2;
    pub const MSG_DONTWAIT: SockMessageFlags = 0x40;
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod os {
    use libc::{c_int, uint8_t};

    pub const AF_UNIX: c_int  = 1;
    pub const AF_LOCAL: c_int = AF_UNIX;
    pub const AF_INET: c_int  = 2;
    pub const AF_INET6: c_int = 30;

    pub const SOCK_STREAM: c_int = 1;
    pub const SOCK_DGRAM: c_int = 2;
    pub const SOCK_SEQPACKET: c_int = 5;
    pub const SOCK_RAW: c_int = 3;
    pub const SOCK_RDM: c_int = 4;

    pub const SOL_SOCKET: c_int = 0xffff;
    pub const IPPROTO_IP: c_int = 0;
    pub const IPPROTO_IPV6: c_int = 41;
    pub const IPPROTO_TCP: c_int = 6;
    pub const IPPROTO_UDP: c_int = 17;

    pub const SO_ACCEPTCONN: c_int          = 0x0002;
    pub const SO_BROADCAST: c_int           = 0x0020;
    pub const SO_DEBUG: c_int               = 0x0001;
    pub const SO_DONTTRUNC: c_int           = 0x2000;
    pub const SO_ERROR: c_int               = 0x1007;
    pub const SO_DONTROUTE: c_int           = 0x0010;
    pub const SO_KEEPALIVE: c_int           = 0x0008;
    pub const SO_LABEL: c_int               = 0x1010;
    pub const SO_LINGER: c_int              = 0x0080;
    pub const SO_NREAD: c_int               = 0x1020;
    pub const SO_NKE: c_int                 = 0x1021;
    pub const SO_NOSIGPIPE: c_int           = 0x1022;
    pub const SO_NOADDRERR: c_int           = 0x1023;
    pub const SO_NOTIFYCONFLICT: c_int      = 0x1026;
    pub const SO_NP_EXTENSIONS: c_int       = 0x1083;
    pub const SO_NWRITE: c_int              = 0x1024;
    pub const SO_OOBINLINE: c_int           = 0x0100;
    pub const SO_PEERLABEL: c_int           = 0x1011;
    pub const SO_RCVBUF: c_int              = 0x1002;
    pub const SO_RCVLOWAT: c_int            = 0x1004;
    pub const SO_SNDLOWAT: c_int            = 0x1003;
    pub const SO_RCVTIMEO: c_int            = 0x1006;
    pub const SO_SNDTIMEO: c_int            = 0x1005;
    pub const SO_RANDOMPORT: c_int          = 0x1082;
    pub const SO_RESTRICTIONS: c_int        = 0x1081;
    pub const SO_RESTRICT_DENYIN: c_int     = 0x00000001;
    pub const SO_RESTRICT_DENYOUT: c_int    = 0x00000002;
    pub const SO_REUSEADDR: c_int           = 0x0004;
    pub const SO_REUSEPORT: c_int           = 0x0200;
    pub const SO_REUSESHAREUID: c_int       = 0x1025;
    pub const SO_SNDBUF: c_int              = 0x1001;
    pub const SO_TIMESTAMP: c_int           = 0x0400;
    pub const SO_TIMESTAMP_MONOTONIC: c_int = 0x0800;
    pub const SO_TYPE: c_int                = 0x1008;
    pub const SO_WANTMORE: c_int            = 0x4000;
    pub const SO_WANTOOBFLAG: c_int         = 0x8000;
    #[allow(overflowing_literals)]
    pub const SO_RESTRICT_DENYSET: c_int    = 0x80000000;

    // Socket options for TCP sockets
    pub const TCP_NODELAY: c_int = 1;
    pub const TCP_MAXSEG: c_int = 2;

    // Socket options for the IP layer of the socket
    pub const IP_MULTICAST_IF: c_int = 9;

    pub type IpMulticastTtl = uint8_t;

    pub const IP_MULTICAST_TTL: c_int = 10;
    pub const IP_MULTICAST_LOOP: c_int = 11;
    pub const IP_ADD_MEMBERSHIP: c_int = 12;
    pub const IP_DROP_MEMBERSHIP: c_int = 13;

    pub type InAddrT = u32;

    // Declarations of special addresses
    pub const INADDR_ANY: InAddrT = 0;
    pub const INADDR_NONE: InAddrT = 0xffffffff;
    pub const INADDR_BROADCAST: InAddrT = 0xffffffff;

    pub type SockMessageFlags = i32;
    // Flags for send/recv and their relatives
    pub const MSG_OOB: SockMessageFlags = 0x1;
    pub const MSG_PEEK: SockMessageFlags = 0x2;
    pub const MSG_DONTWAIT: SockMessageFlags = 0x80;
}

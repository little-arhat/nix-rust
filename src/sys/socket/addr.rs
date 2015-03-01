use {NixResult, NixError, NixPath};
use super::{consts, sa_family_t};
use errno::Errno;
use libc;
use std::{fmt, hash, mem, net, ptr};
use std::ffi::{CStr, OsStr};
use std::num::Int;
use std::path::Path;
use std::os::unix::OsStrExt;

/*
 *
 * ===== AddressFamily =====
 *
 */

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum AddressFamily {
    Unix = consts::AF_UNIX,
    Inet = consts::AF_INET,
    Inet6 = consts::AF_INET6,
}

#[derive(Copy)]
pub enum InetAddr {
    V4(libc::sockaddr_in),
    V6(libc::sockaddr_in6),
}

impl InetAddr {
    pub fn from_std(std: &net::SocketAddr) -> InetAddr {
        InetAddr::new(IpAddr::from_std(&std.ip()), std.port())
    }

    pub fn new(ip: IpAddr, port: u16) -> InetAddr {
        match ip {
            IpAddr::V4(ref ip) => {
                InetAddr::V4(libc::sockaddr_in {
                    sin_family: AddressFamily::Inet as sa_family_t,
                    sin_port: port.to_be(),
                    sin_addr: ip.0,
                    .. unsafe { mem::zeroed() }
                })
            }
            IpAddr::V6(ref ip) => {
                InetAddr::V6(libc::sockaddr_in6 {
                    sin6_family: AddressFamily::Inet6 as sa_family_t,
                    sin6_port: port.to_be(),
                    sin6_addr: ip.0,
                    .. unsafe { mem::zeroed() }
                })
            }
        }
    }

    /// Gets the IP address associated with this socket address.
    pub fn ip(&self) -> IpAddr {
        match *self {
            InetAddr::V4(ref sa) => IpAddr::V4(Ipv4Addr(sa.sin_addr)),
            InetAddr::V6(ref sa) => IpAddr::V6(Ipv6Addr(sa.sin6_addr)),
        }
    }

    /// Gets the port number associated with this socket address
    pub fn port(&self) -> u16 {
        match *self {
            InetAddr::V6(ref sa) => Int::from_be(sa.sin6_port),
            InetAddr::V4(ref sa) => Int::from_be(sa.sin_port),
        }
    }

    pub fn to_std(&self) -> net::SocketAddr {
        net::SocketAddr::new(self.ip().to_std(), self.port())
    }

    pub fn to_str(&self) -> String {
        format!("{}", self)
    }
}

impl PartialEq for InetAddr {
    fn eq(&self, other: &InetAddr) -> bool {
        match (*self, *other) {
            (InetAddr::V4(ref a), InetAddr::V4(ref b)) => {
                a.sin_port == b.sin_port &&
                    a.sin_addr.s_addr == b.sin_addr.s_addr
            }
            (InetAddr::V6(ref a), InetAddr::V6(ref b)) => {
                a.sin6_port == b.sin6_port &&
                    a.sin6_addr.s6_addr == b.sin6_addr.s6_addr &&
                    a.sin6_flowinfo == b.sin6_flowinfo &&
                    a.sin6_scope_id == b.sin6_scope_id
            }
            _ => false,
        }
    }
}

impl Eq for InetAddr {
}

impl hash::Hash for InetAddr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        match *self {
            InetAddr::V4(ref a) => {
                ( a.sin_family,
                  a.sin_port,
                  a.sin_addr.s_addr ).hash(s)
            }
            InetAddr::V6(ref a) => {
                ( a.sin6_family,
                  a.sin6_port,
                  &a.sin6_addr.s6_addr,
                  a.sin6_flowinfo,
                  a.sin6_scope_id ).hash(s)
            }
        }
    }
}

impl Clone for InetAddr {
    fn clone(&self) -> InetAddr {
        *self
    }
}

impl fmt::Display for InetAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InetAddr::V4(_) => write!(f, "{}:{}", self.ip(), self.port()),
            InetAddr::V6(_) => write!(f, "[{}]:{}", self.ip(), self.port()),
        }
    }
}

/*
 *
 * ===== IpAddr =====
 *
 */

pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddr {
    /// Create a new IpAddr that contains an IPv4 address.
    ///
    /// The result will represent the IP address a.b.c.d
    pub fn new_v4(a: u8, b: u8, c: u8, d: u8) -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(a, b, c, d))
    }

    /// Create a new IpAddr that contains an IPv6 address.
    ///
    /// The result will represent the IP address a:b:c:d:e:f
    pub fn new_v6(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> IpAddr {
        IpAddr::V6(Ipv6Addr::new(a, b, c, d, e, f, g, h))
    }

    pub fn from_std(std: &net::IpAddr) -> IpAddr {
        match *std {
            net::IpAddr::V4(ref std) => IpAddr::V4(Ipv4Addr::from_std(std)),
            net::IpAddr::V6(ref std) => IpAddr::V6(Ipv6Addr::from_std(std)),
        }
    }

    pub fn to_std(&self) -> net::IpAddr {
        match *self {
            IpAddr::V4(ref ip) => net::IpAddr::V4(ip.to_std()),
            IpAddr::V6(ref ip) => net::IpAddr::V6(ip.to_std()),
        }
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpAddr::V4(ref v4) => v4.fmt(f),
            IpAddr::V6(ref v6) => v6.fmt(f)
        }
    }
}

/*
 *
 * ===== Ipv4Addr =====
 *
 */

#[derive(Copy)]
pub struct Ipv4Addr(pub libc::in_addr);

impl Ipv4Addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        let ip = (((a as u32) << 24) |
                  ((b as u32) << 16) |
                  ((c as u32) <<  8) |
                  ((d as u32) <<  0)).to_be();

        Ipv4Addr(libc::in_addr { s_addr: ip })
    }

    pub fn from_std(std: &net::Ipv4Addr) -> Ipv4Addr {
        let bits = std.octets();
        Ipv4Addr::new(bits[0], bits[1], bits[2], bits[3])
    }

    pub fn any() -> Ipv4Addr {
        Ipv4Addr(libc::in_addr { s_addr: consts::INADDR_ANY })
    }

    pub fn octets(&self) -> [u8; 4] {
        let bits = Int::from_be(self.0.s_addr);
        [(bits >> 24) as u8, (bits >> 16) as u8, (bits >> 8) as u8, bits as u8]
    }

    pub fn to_std(&self) -> net::Ipv4Addr {
        let bits = self.octets();
        net::Ipv4Addr::new(bits[0], bits[1], bits[2], bits[3])
    }
}

impl PartialEq for Ipv4Addr {
    fn eq(&self, other: &Ipv4Addr) -> bool {
        self.0.s_addr == other.0.s_addr
    }
}

impl Eq for Ipv4Addr {
}

impl hash::Hash for Ipv4Addr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.0.s_addr.hash(s)
    }
}

impl Clone for Ipv4Addr {
    fn clone(&self) -> Ipv4Addr {
        *self
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let octets = self.octets();
        write!(fmt, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
    }
}

/*
 *
 * ===== Ipv6Addr =====
 *
 */

#[derive(Copy)]
pub struct Ipv6Addr(pub libc::in6_addr);

impl Ipv6Addr {
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr {
        Ipv6Addr(libc::in6_addr {
            s6_addr: [
                a.to_be(),
                b.to_be(),
                c.to_be(),
                d.to_be(),
                e.to_be(),
                f.to_be(),
                g.to_be(),
                h.to_be(),
            ]
        })
    }

    pub fn from_std(std: &net::Ipv6Addr) -> Ipv6Addr {
        let s = std.segments();
        Ipv6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }

    /// Return the eight 16-bit segments that make up this address
    pub fn segments(&self) -> [u16; 8] {
        [Int::from_be(self.0.s6_addr[0]),
         Int::from_be(self.0.s6_addr[1]),
         Int::from_be(self.0.s6_addr[2]),
         Int::from_be(self.0.s6_addr[3]),
         Int::from_be(self.0.s6_addr[4]),
         Int::from_be(self.0.s6_addr[5]),
         Int::from_be(self.0.s6_addr[6]),
         Int::from_be(self.0.s6_addr[7])]
    }

    pub fn to_std(&self) -> net::Ipv6Addr {
        let s = self.segments();
        net::Ipv6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }
}

impl fmt::Display for Ipv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.to_std().fmt(fmt)
    }
}

/*
 *
 * ===== UnixAddr =====
 *
 */

#[derive(Copy)]
pub struct UnixAddr(pub libc::sockaddr_un);

impl UnixAddr {
    pub fn new<P: ?Sized + NixPath>(path: &P) -> NixResult<UnixAddr> {
        try!(path.with_nix_path(|osstr| {
            unsafe {
                let bytes = osstr.as_bytes();

                let mut ret = libc::sockaddr_un {
                    sun_family: AddressFamily::Unix as sa_family_t,
                    .. mem::zeroed()
                };

                if bytes.len() >= ret.sun_path.len() {
                    return Err(NixError::Sys(Errno::ENAMETOOLONG));
                }

                ptr::copy_memory(
                    ret.sun_path.as_mut_ptr(),
                    bytes.as_ptr() as *const i8,
                    bytes.len());

                Ok(UnixAddr(ret))
            }
        }))
    }

    pub fn path(&self) -> &Path {
        unsafe {
            let bytes = CStr::from_ptr(self.0.sun_path.as_ptr()).to_bytes();
            Path::new(<OsStr as OsStrExt>::from_bytes(bytes))
        }
    }
}

impl PartialEq for UnixAddr {
    fn eq(&self, other: &UnixAddr) -> bool {
        unsafe {
            0 == libc::strcmp(self.0.sun_path.as_ptr(), other.0.sun_path.as_ptr())
        }
    }
}

impl Eq for UnixAddr {
}

impl hash::Hash for UnixAddr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        ( self.0.sun_family, self.path() ).hash(s)
    }
}

impl Clone for UnixAddr {
    fn clone(&self) -> UnixAddr {
        *self
    }
}

impl fmt::Display for UnixAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.path().display().fmt(f)
    }
}

/*
 *
 * ===== Sock addr =====
 *
 */

/// Represents a socket address
#[derive(Copy)]
pub enum SockAddr {
    Inet(InetAddr),
    Unix(UnixAddr)
}

impl SockAddr {
    pub fn new_inet(addr: InetAddr) -> SockAddr {
        SockAddr::Inet(addr)
    }

    pub fn new_unix<P: ?Sized + NixPath>(path: &P) -> NixResult<SockAddr> {
        Ok(SockAddr::Unix(try!(UnixAddr::new(path))))
    }

    pub fn family(&self) -> AddressFamily {
        match *self {
            SockAddr::Inet(InetAddr::V4(..)) => AddressFamily::Inet,
            SockAddr::Inet(InetAddr::V6(..)) => AddressFamily::Inet6,
            SockAddr::Unix(..) => AddressFamily::Unix,
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}", self)
    }

    pub unsafe fn as_ffi_pair(&self) -> (&libc::sockaddr, libc::socklen_t) {
        match *self {
            SockAddr::Inet(InetAddr::V4(ref addr)) => (mem::transmute(addr), mem::size_of::<libc::sockaddr_in>() as libc::socklen_t),
            SockAddr::Inet(InetAddr::V6(ref addr)) => (mem::transmute(addr), mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t),
            SockAddr::Unix(UnixAddr(ref addr)) => (mem::transmute(addr), mem::size_of::<libc::sockaddr_un>() as libc::socklen_t),
        }
    }
}

impl PartialEq for SockAddr {
    fn eq(&self, other: &SockAddr) -> bool {
        match (*self, *other) {
            (SockAddr::Inet(ref a), SockAddr::Inet(ref b)) => {
                a == b
            }
            (SockAddr::Unix(ref a), SockAddr::Unix(ref b)) => {
                a == b
            }
            _ => false,
        }
    }
}

impl Eq for SockAddr {
}

impl hash::Hash for SockAddr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        match *self {
            SockAddr::Inet(ref a) => a.hash(s),
            SockAddr::Unix(ref a) => a.hash(s),
        }
    }
}

impl Clone for SockAddr {
    fn clone(&self) -> SockAddr {
        *self
    }
}

impl fmt::Display for SockAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SockAddr::Inet(ref inet) => inet.fmt(f),
            SockAddr::Unix(ref unix) => unix.fmt(f),
        }
    }
}

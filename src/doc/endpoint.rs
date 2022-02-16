//! Currently the CAS has two endpoint: [`DirectEndpoint`] and [`WebVPNEndpoint`].
//!
//! To use [`WebVPNEndpoint`], please enable feature **webvpn**.
#![allow(missing_debug_implementations)]

/// Access CAS directly.
///
/// Specifically, access through domain `pass.neu.edu.cn`.
///
/// ```plain
///  +--------------+       3.Verify       +--------------+
///  |              +<---------------------+              |
///  |     CAS      |                      |              |
///  |              +--------------------->+              |
///  +------+-------+         4.Ok         |   Services   |
///         ^                              |              |
///         |                              |   connected  |
/// 1.Login |                              |              |
///         |                              |      to      |
///         |                              |              |
///  +------+-------+       5.Service      |      CAS     |
///  |              +<---------------------+              |
///  |     User     |                      |              |
///  |              +--------------------->+              |
///  +--------------+       2. visit       +--------------+
/// ```
pub struct DirectEndpoint;

/// Access CAS via WebVPN.
///
/// Specifically, access through domain `webvpn.neu.edu.cn`.
///
/// This endpoint is used when programs need to access intranet services connected to CAS.
///
/// Note that the WebVPN is both a service connected to CAS and
/// a proxy managing cookies from proxied services.
///
/// When we access services via WebVPN, it plays the role of user (see Proxy Box below).
///
/// So we are anonymous to those services if we haven't login CAS via WebVPN, for the CAS
/// can not identify who the proxy (a virtual user) is.
///
/// That's why we should login CAS directly first and then login CAS via WebVPN.
///
/// ```plain
///                    8.Ok                +---------------------------+
///       +------------------------------->+                           |
///       |                                | Services connected to CAS |
///       |       +------------------------+                           |
///       |       |          7.Verify      +----------+-------+--------+
///       |       |                                   ^       |
///       |       |                                   |       |
///       |       v                            6.Visit|       |9.Service
/// +-----+-------+----+                              |       |
/// |                  |                              |       v
/// |                  |                       +------+-------+--------+
/// |       CAS        |        5.Login        |                       |
/// |                  +<----------------------+  Proxy(virtual user)  |
/// |                  |                       |                       |
/// +--------+---+--+--+                       +------+-------+--------+
///          ^   ^  |                                 ^       |
///          |   |  |                                 |       v
///          |   |  |          4.Ok            +------+-------+--------+
///          |   |  +------------------------->+                       |
///   1.Login|   |         3. Verify           |         WebVPN        |
///          |   +-----------------------------+                       |
///          |                                 +--------+----+---------+
///          |                                          ^    |
/// +--------+---------+ 2.Visit services via WebVPN    |    |
/// |                  +--------------------------------+    |
/// |       User       |                                     |
/// |                  +<------------------------------------+
/// +------------------+          10.Service
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "webvpn")))]
pub struct WebVPNEndpoint;

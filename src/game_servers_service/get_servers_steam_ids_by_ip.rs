//! Implements the 'GetServerSteamIDsByIP' endpoint.
//!
//! This endpoint is poorly explained in the steam docs.
//! The server that is sent through the "server_ips" argument really refers to
//! severs linked to the key (set up with `Steam::new(key)`) basically only linked
//! servers can be polled, if you have these servers they should be also viewable
//! through the function `get_account_list` as the `servers` field.

// Currently I don't have access to something to run a steam server on so I can't test the returns of the endpoint
// and since steam doesn't offer an example return it is a bit hard to develop this code, this should be fixable by
// creating my own server which will be possible soon.

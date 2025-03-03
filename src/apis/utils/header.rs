/// A collection of key-value pairs representing HTTP headers
type Header = Vec<(String, String)>;

// pub struct Header {
// host: string,
// x-amz-Date: <Date>
// Authorization: AWS4-HMAC-SHA256 Credential=<Credential>, SignedHeaders=content-type;date;host;user-agent;x-amz-date;x-amz-target;x-amzn-requestid, Signature=<Signature>
// User-Agent: <UserAgentString>
// Content-Type: application/x-amz-json-1.1
// Content-Length: <PayloadSizeBytes>
// Connection: Keep-Alive
// X-Amz-Target: AWSEvents.PutEvents
// }

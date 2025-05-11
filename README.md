<div style="text-align: center">
<p style="text-align: center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 style="text-align: center">Amazon EventBridge component for Edgee</h1>

This component provides a seamless integration between [Edgee](https://www.edgee.cloud/) and [Amazon EventBridge](https://aws.amazon.com/eventbridge/). Allowing users to send events.

---
## Setup
1. Requirements:
   - cargo
   - wit-deps
   - [edgee-cli](https://github.com/edgee-cloud/edgee)
1. Place the s3.wasm file in your server (e.g., `/var/edgee/components`)
1. Add the following configuration to your edgee.toml:

```toml
[[destinations.data_collection]]
id = "amazon-event-bridge"
file = "/var/edgee/components/event_bridge.wasm"
ettings.aws_region = "YOUR_AWS_REGION"
settings.aws_domain = "YOUR_AWS_DOMAIN"
settings.aws_access_key = "YOUR_AWS_ACCESS_KEY"
settings.aws_secret_key = "YOUR_AWS_SECRET_KEY"
settings.aws_security_token = "YOUR_AWS_SECURITY_TOKEN(OPTIONAL)"
```


## Building

```shell
$ make build
```

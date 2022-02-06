# Readme

Source of code: https://actix.rs/docs/getting-started/

# Benchmarking performance:

Why I am so curious about performance noww..?

Reason: https://www.techempower.com/benchmarks/#section=data-r18

```bash
# Installing autocannot, a benchmarking tool for servers:
sudo npm i -g autocannon

# Running server and benchmarking:
cargo run --release # This would run in production profile instead of running ``cargo run``.
autocannon -c 15000 -d 5 localhost:8080
```

Although this link: https://www.techempower.com/benchmarks/#section=data-r18 shows that actix-core has very powerful request per second dealing but in my experience its only capable of handling at `15k concurrent requests` in an average of `8 seconds` where as nodejs handles same scenario in an average of `14 seconds`.

Another helper article rust benchmarking: https://klau.si/blog/benchmarking-a-rust-web-application/

FROM	rust:1.70

COPY	./ ./

RUN		rustup override set nightly
RUN		cargo build

CMD		["cargo", "run"]

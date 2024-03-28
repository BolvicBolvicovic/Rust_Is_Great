FROM	rust:1.70

COPY	./ ./

RUN		rustup override set nightly
RUN		cargo build

EXPOSE	8000

CMD		["cargo", "run"]

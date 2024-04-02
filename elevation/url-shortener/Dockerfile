FROM	rust:1.74

COPY	./ ./

RUN		rustup override set nightly
RUN		cargo build

EXPOSE	8000

CMD		["cargo", "run"]

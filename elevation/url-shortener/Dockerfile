FROM	rust:1.74

COPY	./ ./

RUN		rustup override set nightly
RUN		cargo build

ENV		PORT=443
EXPOSE	443

CMD		["cargo", "run"]

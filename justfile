work day part:
    cargo watch --ignore 'day-*.bench.txt' -x "check -p {{day}}" -s "just test {{part}} -p {{day}}" -s "just lint {{day}}" -s "just bench {{day}} {{part}}"
lint day:
    cargo clippy -p {{day}}
test part +FLAGS='-p day-01':
    cargo test {{FLAGS}} {{part}}
bench-all:
    cargo bench -q > benchmarks.txt
bench day part:
    cargo bench --bench {{day}} {{part}} >> {{day}}.bench.txt
create day:
    cargo generate --path ./daily-template --name {{day}}
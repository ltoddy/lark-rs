test:export APP_ID=
test:export APP_SECRET=
test:
	@cargo test --quiet -- --nocapture

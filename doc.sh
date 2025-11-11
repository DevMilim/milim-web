cd core
cargo doc --no-deps --document-private-items
cd ..
rm -rf doc/**
cp -r target/doc/* ./docs
cat > docs/index.html <<'HTML'
<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Docs</title>
    <meta http-equiv="refresh" content="0; url=core/index.html">
  </head>
  <body>
    <p>Redirecionando para <a href="core/index.html">core docs</a>…</p>
  </body>
</html>
HTML
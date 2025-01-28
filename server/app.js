const express = require('express');
const app = express();
const port = 3000;

// Import routes
const routes = require('./routes');

// Use routes
app.use('/api', routes);

// Start server
app.listen(port, () => {
  console.log(`Server started on port ${port}`);
});


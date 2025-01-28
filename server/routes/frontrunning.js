const express = require('express');
const router = express.Router();

// Import frontrunning logic
const frontrunningLogic = require('../../logic/frontrunning');

// Define route handlers
router.get('/', async (req, res) => {
  try {
    const data = await frontrunningLogic.getFrontrunningData();
    res.json(data);
  } catch (error) {
    res.status(500).json({ error: 'Failed to get frontrunning data' });
  }
});

// Export router
module.exports = router;

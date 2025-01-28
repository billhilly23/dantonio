const express = require('express');
const router = express.Router();

// Import flash loan logic
const flashloanLogic = require('../../logic/flashloan');

// Define route handlers
router.get('/', async (req, res) => {
  try {
    const data = await flashloanLogic.getFlashloanData();
    res.json(data);
  } catch (error) {
    res.status(500).json({ error: 'Failed to get flash loan data' });
  }
});

// Export router
module.exports = router;


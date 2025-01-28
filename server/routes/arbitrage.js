const express = require('express');
const router = express.Router();

// Import arbitrage logic
const arbitrageLogic = require('../../logic/arbitrage');

// Define route handlers
router.get('/', async (req, res) => {
  try {
    const data = await arbitrageLogic.getArbitrageData();
    res.json(data);
  } catch (error) {
    res.status(500).json({ error: 'Failed to get arbitrage data' });
  }
});

// Export router
module.exports = router;

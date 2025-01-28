const express = require('express');
const router = express.Router();

// Import liquidation logic
const liquidationLogic = require('../../logic/liquidation');

// Define route handlers
router.get('/', async (req, res) => {
  try {
    const data = await liquidationLogic.getLiquidationData();
    res.json(data);
  } catch (error) {
    res.status(500).json({ error: 'Failed to get liquidation data' });
  }
});

// Export router
module.exports = router;

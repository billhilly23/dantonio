const express = require('express');
const router = express.Router();

// Import HFT logic
const hftLogic = require('../../logic/hft');

// Define route handlers
router.get('/', async (req, res) => {
  try {
    const data = await hftLogic.getHftData();
    res.json(data);
  } catch (error) {
    res.status(500).json({ error: 'Failed to get HFT data' });
  }
});

// Export router
module.exports = router;

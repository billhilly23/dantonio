const express = require('express');
const router = express.Router();

// Import sandwich logic
const sandwichLogic = require('../../logic/sandwich');

// Define route handlers
router.get('/', async (req, res) => {
  try {
    const data = await sandwichLogic.getSandwichData();
    res.json(data);
  } catch (error) {
    res.status(500).json({ error: 'Failed to get sandwich data' });
  }
});

// Export router
module.exports = router;

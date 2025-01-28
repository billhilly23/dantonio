const express = require('express');
const router = express.Router();

// Import dashboard logic
const dashboardLogic = require('../../logic/dashboard');

// Define route handlers
router.get('/', async (req, res) => {
  try {
    const data = await dashboardLogic.getDashboardData();
    res.json(data);
  } catch (error) {
    res.status(500).json({ error: 'Failed to get dashboard data' });
  }
});

// Export router
module.exports = router;

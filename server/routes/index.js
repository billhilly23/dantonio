const express = require('express');
const router = express.Router();

// Import route handlers
const dashboardRoutes = require('./dashboard');
const frontrunningRoutes = require('./frontrunning');
const hftRoutes = require('./hft');
const liquidationRoutes = require('./liquidation');
const sandwichRoutes = require('./sandwich');
const flashloanRoutes = require('./flashloan');

// Use route handlers
router.use('/dashboard', dashboardRoutes);
router.use('/frontrunning', frontrunningRoutes);
router.use('/hft', hftRoutes);
router.use('/liquidation', liquidationRoutes);
router.use('/sandwich', sandwichRoutes);
router.use('/flashloan', flashloanRoutes);

// Export router
module.exports = router;

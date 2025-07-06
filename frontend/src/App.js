import React from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import SponsorPositionCreation from './pages/SponsorPositionCreation';
import Positions from './pages/Positions';


function App() {
  return (
    <Router>
      <div className="App">
        {/* Navigation */}
        <nav className="bg-blue-600 text-white p-4">
          <div className="max-w-6xl mx-auto">
            <Link to="/" className="text-xl font-bold">YieldBack.Cash</Link>
            <div className="mt-2">
              <Link to="/" className="mr-4 hover:underline">About</Link>
              <Link to="/positions" className="mr-4 hover:underline">Positions</Link>
              <Link to="/sponsor-position" className="hover:underline">Sponsor</Link>
            </div>
          </div>
        </nav>

        {/* Routes */}
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/positions" element={<Positions />} />
          <Route path="/sponsor-position" element={<SponsorPositionCreation />} />
        </Routes>
      </div>
    </Router>
  );
}

// Simple Home component
function Home() {
  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4">
      <div className="max-w-4xl mx-auto text-center">
        <h1 className="text-4xl font-bold text-gray-900 mb-4">YieldBack.Cash</h1>
        <p className="text-xl text-gray-600 mb-8">
          Decentralized fixed income bond market
        </p>
        <div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
          <Link 
            to="/positions" 
            className="bg-green-600 text-white px-6 py-3 rounded-md hover:bg-green-700 transition duration-200 inline-block"
          >
            Browse Positions
          </Link>
          <Link 
            to="/sponsor-position" 
            className="bg-blue-600 text-white px-6 py-3 rounded-md hover:bg-blue-700 transition duration-200 inline-block"
          >
            Sponsor a position
          </Link>
        </div>

        <div className="max-w-3xl mx-auto text-left mt-20">
          <h2 className="text-2xl font-semibold text-gray-800 mb-6 text-center">How YieldBack.Cash Works</h2>
          
          <div className="grid md:grid-cols-2 gap-8">
            <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
              <div className="flex items-center mb-3">
                <div className="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center mr-3">
                  <span className="text-blue-600 font-semibold">1</span>
                </div>
                <h3 className="text-lg font-medium text-gray-900">For Sponsors</h3>
              </div>
              <p className="text-gray-600">
                Deposit coupon amount, determine maturity period and user's principal.
              </p>
            </div>

            <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
              <div className="flex items-center mb-3">
                <div className="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center mr-3">
                  <span className="text-green-600 font-semibold">2</span>
                </div>
                <h3 className="text-lg font-medium text-gray-900">For Users</h3>
              </div>
              <p className="text-gray-600">
                View available positions to deposit principal payment. Earn fixed income.
              </p>
            </div>
          </div>

          <div className="mt-8 bg-gray-50 p-6 rounded-lg">
            <h3 className="text-lg font-medium text-gray-900 mb-3 text-center">Benefits of YieldBack.Cash</h3>
            <div className="grid sm:grid-cols-3 gap-4 text-center">
              <div>
                <div className="text-blue-600 font-semibold mb-1">Secure</div>
                <p className="text-sm text-gray-600">Smart contracts on Stellar ensure transparent and secure transactions</p>
              </div>
              <div>
                <div className="text-green-600 font-semibold mb-1">Low cost</div>
                <p className="text-sm text-gray-600">Low transaction fees on the Stellar blockchain</p>
              </div>
              <div>
                <div className="text-purple-600 font-semibold mb-1">Decentralized</div>
                <p className="text-sm text-gray-600">Logic and payments handled in a decentralized manner using Soroban smart contracts</p>
              </div>
            </div>
          </div>
        </div>
      </div>
      </div>
  );
}

export default App;
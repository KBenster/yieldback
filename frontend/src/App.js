import React from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import SponsorPositionCreation from './pages/SponsorPositionCreation';



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
              <Link to="/sponsor-position" className="hover:underline">Sponsor</Link>
            </div>
          </div>
        </nav>

        {/* Routes */}
        <Routes>
          <Route path="/" element={<Home />} />
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
        <h1 className="text-4xl font-bold text-gray-900 mb-4">Welcome</h1>
        <Link 
          to="/sponsor-position" 
          className="bg-blue-600 text-white px-6 py-3 rounded-md hover:bg-blue-700 transition duration-200"
        >
          Create Sponsor Position
        </Link>
      </div>
    </div>
  );
}

export default App;
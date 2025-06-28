import React, { useState } from 'react';

export default function SponsorPositionCreation() {
  const [formData, setFormData] = useState({
    coupon: '',
    userPrincipal: '',
    daysUntilMaturity: ''
  });

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
  };

  const handleSubmit = () => {
    console.log('Form submitted:', formData);
    // Handle form submission logic here
  };

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md mx-auto bg-white rounded-lg shadow-md p-6">
        <h1 className="text-2xl font-bold text-gray-900 text-center mb-8">
          Sponsor Position Creation
        </h1>
        
        <div className="space-y-6">
          <div>
            <label htmlFor="coupon" className="block text-sm font-medium text-gray-700 mb-2">
              Coupon
            </label>
            <input
              type="number"
              id="coupon"
              name="coupon"
              value={formData.coupon}
              onChange={handleInputChange}
              step="0.01"
              min="0"
              className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              placeholder="Enter coupon value"
            />
          </div>

          <div>
            <label htmlFor="userPrincipal" className="block text-sm font-medium text-gray-700 mb-2">
              User Principal
            </label>
            <input
              type="number"
              id="userPrincipal"
              name="userPrincipal"
              value={formData.userPrincipal}
              onChange={handleInputChange}
              step="0.01"
              min="0"
              className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              placeholder="Enter user principal"
            />
          </div>

          <div>
            <label htmlFor="daysUntilMaturity" className="block text-sm font-medium text-gray-700 mb-2">
              Days Until Maturity
            </label>
            <input
              type="number"
              id="daysUntilMaturity"
              name="daysUntilMaturity"
              value={formData.daysUntilMaturity}
              onChange={handleInputChange}
              min="1"
              className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              placeholder="Enter number of days"
            />
          </div>

          <button
            onClick={handleSubmit}
            className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition duration-200 font-medium"
          >
            Create Position
          </button>
        </div>

        <div className="mt-8 p-4 bg-gray-100 rounded-md">
          <h3 className="text-sm font-medium text-gray-700 mb-2">Current Values:</h3>
          <div className="text-sm text-gray-600 space-y-1">
            <div>Coupon: {formData.coupon || 'Not set'}</div>
            <div>User Principal: {formData.userPrincipal || 'Not set'}</div>
            <div>Days Until Maturity: {formData.daysUntilMaturity || 'Not set'}</div>
          </div>
        </div>
      </div>
    </div>
  );
}
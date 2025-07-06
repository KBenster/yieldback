import React from 'react';

export default function Positions() {
  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-4">
            Available Positions
          </h1>
          <p className="text-lg text-gray-600">
            Browse and match with bond positions
          </p>
        </div>

        {/* Placeholder content */}
        <div className="bg-white rounded-lg shadow-md p-8 text-center">
          <div className="max-w-md mx-auto">
            <div className="mb-4">
              <svg
                className="mx-auto h-12 w-12 text-gray-400"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                aria-hidden="true"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                />
              </svg>
            </div>
            <h3 className="text-lg font-medium text-gray-900 mb-2">
              No Positions Available Yet
            </h3>
            <p className="text-gray-500 mb-6">
              Position matching functionality is coming soon. Check back later to see available sponsor positions you can match with.
            </p>
            <div className="text-sm text-gray-400">
              This page will show:
              <ul className="mt-2 space-y-1">
                <li>• Available sponsor positions</li>
                <li>• Match compatibility scores</li>
                <li>• Position details and terms</li>
                <li>• One-click matching</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
import axios from 'axios';

// TODO: Get API base URL from environment variables
const API_BASE_URL =
  process.env.REACT_APP_API_URL || 'http://localhost:8000/api'; // Default for local dev

const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Optional: Add interceptors for handling auth tokens, errors, etc.
// apiClient.interceptors.request.use(config => {
//   const token = localStorage.getItem('authToken'); // Example: Get token from storage
//   if (token) {
//     config.headers.Authorization = `Bearer ${token}`;
//   }
//   return config;
// });

// apiClient.interceptors.response.use(
//   response => response,
//   error => {
//     // Handle errors globally (e.g., redirect on 401)
//     console.error('API Error:', error.response || error.message);
//     return Promise.reject(error);
//   }
// );

export default apiClient;

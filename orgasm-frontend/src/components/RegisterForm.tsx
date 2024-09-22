import React, { useState } from 'react';
import { Button } from './Button';

interface RegisterFormProps {
  onClose: () => void;
}

const RegisterForm: React.FC<RegisterFormProps> = ({ onClose }) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    // TODO: Implement actual registration logic here
    console.log('Registering with:', { username, password });
    // For now, we'll just close the form after submission
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
      <div className="bg-background p-6 rounded-lg shadow-lg">
        <h2 className="text-2xl font-bold mb-4">Register</h2>
        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label htmlFor="username" className="block mb-2">Username</label>
            <input
              type="text"
              id="username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              className="w-full p-2 border rounded"
              required
            />
          </div>
          <div className="mb-4">
            <label htmlFor="password" className="block mb-2">Password</label>
            <input
              type="password"
              id="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full p-2 border rounded"
              required
            />
          </div>
          <div className="flex justify-end">
            <Button type="button" variant="secondary" onClick={onClose} className="mr-2">
              Cancel
            </Button>
            <Button type="submit">Register</Button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default RegisterForm;
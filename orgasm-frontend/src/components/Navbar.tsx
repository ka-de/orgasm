import React from 'react';
import { Button } from "./Button"

const Navbar: React.FC = () => {
  return (
    <nav className="bg-primary text-primary-foreground">
      <div className="container mx-auto px-4 py-2 flex justify-between items-center">
        <h1 className="text-2xl font-bold">Orgasm Imageboard</h1>
        <div>
          <Button variant="secondary" className="mr-2">Login</Button>
          <Button>Sign Up</Button>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
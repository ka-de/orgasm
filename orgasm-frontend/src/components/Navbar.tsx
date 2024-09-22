import React from 'react';
import { Button } from "./Button"

interface NavbarProps {
  onSignUpClick: () => void;
}

const Navbar: React.FC<NavbarProps> = ({ onSignUpClick }) => {
  return (
    <nav className="bg-primary text-primary-foreground">
      <div className="px-4 py-2 flex justify-between items-center">
        <h1 className="text-2xl font-bold">Orgasm Imageboard</h1>
        <div>
          <Button variant="secondary" className="mr-2">Login</Button>
          <Button onClick={onSignUpClick}>Sign Up</Button>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
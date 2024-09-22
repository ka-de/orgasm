import React from 'react';
import Navbar from './Navbar';

interface LayoutProps {
  children: React.ReactNode;
  onSignUpClick: () => void;
}

const Layout: React.FC<LayoutProps> = ({ children, onSignUpClick }) => {
  return (
    <div className="min-h-screen flex flex-col bg-background text-foreground">
      <Navbar onSignUpClick={onSignUpClick} />
      <main className="flex-grow px-4 py-8 w-full">
        {children}
      </main>
    </div>
  );
};

export default Layout;
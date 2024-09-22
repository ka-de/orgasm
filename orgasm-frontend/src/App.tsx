import React from 'react';
import Layout from './components/Layout';
import ImageGrid from './components/ImageGrid';

const App: React.FC = () => {
  // Mock data for images
  const images = [
    { id: 1, url: 'https://picsum.photos/400/300?random=1', title: 'Image 1' },
    { id: 2, url: 'https://picsum.photos/400/300?random=2', title: 'Image 2' },
    { id: 3, url: 'https://picsum.photos/400/300?random=3', title: 'Image 3' },
    { id: 4, url: 'https://picsum.photos/400/300?random=4', title: 'Image 4' },
    { id: 5, url: 'https://picsum.photos/400/300?random=5', title: 'Image 5' },
    { id: 6, url: 'https://picsum.photos/400/300?random=6', title: 'Image 6' },
  ];

  return (
    <Layout>
      <h2 className="text-3xl font-bold mb-6">Latest Images</h2>
      <ImageGrid images={images} />
    </Layout>
  );
};

export default App;
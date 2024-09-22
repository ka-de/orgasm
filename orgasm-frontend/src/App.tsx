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
    { id: 7, url: 'https://picsum.photos/400/300?random=7', title: 'Image 7' },
    { id: 8, url: 'https://picsum.photos/400/300?random=8', title: 'Image 8' },
    { id: 9, url: 'https://picsum.photos/400/300?random=9', title: 'Image 9' },
    { id: 10, url: 'https://picsum.photos/400/300?random=10', title: 'Image 10' },
    { id: 11, url: 'https://picsum.photos/400/300?random=11', title: 'Image 11' },
    { id: 12, url: 'https://picsum.photos/400/300?random=12', title: 'Image 12' },
    { id: 13, url: 'https://picsum.photos/400/300?random=13', title: 'Image 13' },
    { id: 14, url: 'https://picsum.photos/400/300?random=14', title: 'Image 14' },
    { id: 15, url: 'https://picsum.photos/400/300?random=15', title: 'Image 15' },
    { id: 16, url: 'https://picsum.photos/400/300?random=16', title: 'Image 16' },
    { id: 17, url: 'https://picsum.photos/400/300?random=17', title: 'Image 17' },
    { id: 18, url: 'https://picsum.photos/400/300?random=18', title: 'Image 18' },
    { id: 19, url: 'https://picsum.photos/400/300?random=19', title: 'Image 19' },
    { id: 20, url: 'https://picsum.photos/400/300?random=20', title: 'Image 20' },
    { id: 21, url: 'https://picsum.photos/400/300?random=21', title: 'Image 21' },
    { id: 22, url: 'https://picsum.photos/400/300?random=22', title: 'Image 22' },
    { id: 23, url: 'https://picsum.photos/400/300?random=23', title: 'Image 23' },
    { id: 24, url: 'https://picsum.photos/400/300?random=24', title: 'Image 24' },
    { id: 25, url: 'https://picsum.photos/400/300?random=25', title: 'Image 25' },
    { id: 26, url: 'https://picsum.photos/400/300?random=26', title: 'Image 26' },
    { id: 27, url: 'https://picsum.photos/400/300?random=27', title: 'Image 27' },
    { id: 28, url: 'https://picsum.photos/400/300?random=28', title: 'Image 28' },
    { id: 29, url: 'https://picsum.photos/400/300?random=29', title: 'Image 29' },
    { id: 30, url: 'https://picsum.photos/400/300?random=30', title: 'Image 30' },
    { id: 31, url: 'https://picsum.photos/400/300?random=31', title: 'Image 31' },
    { id: 32, url: 'https://picsum.photos/400/300?random=32', title: 'Image 32' },
    { id: 33, url: 'https://picsum.photos/400/300?random=33', title: 'Image 33' },
    { id: 34, url: 'https://picsum.photos/400/300?random=34', title: 'Image 34' },
    { id: 35, url: 'https://picsum.photos/400/300?random=35', title: 'Image 35' },
    { id: 36, url: 'https://picsum.photos/400/300?random=36', title: 'Image 36' },
    { id: 37, url: 'https://picsum.photos/400/300?random=37', title: 'Image 37' },
    { id: 38, url: 'https://picsum.photos/400/300?random=38', title: 'Image 38' },
    { id: 39, url: 'https://picsum.photos/400/300?random=39', title: 'Image 39' },
    { id: 40, url: 'https://picsum.photos/400/300?random=40', title: 'Image 40' },
    { id: 41, url: 'https://picsum.photos/400/300?random=41', title: 'Image 41' },
    { id: 42, url: 'https://picsum.photos/400/300?random=42', title: 'Image 42' },
    { id: 43, url: 'https://picsum.photos/400/300?random=43', title: 'Image 43' },
    { id: 44, url: 'https://picsum.photos/400/300?random=44', title: 'Image 44' },
    { id: 45, url: 'https://picsum.photos/400/300?random=45', title: 'Image 45' },
    { id: 46, url: 'https://picsum.photos/400/300?random=46', title: 'Image 46' },
    { id: 47, url: 'https://picsum.photos/400/300?random=47', title: 'Image 47' },
    { id: 48, url: 'https://picsum.photos/400/300?random=48', title: 'Image 48' },
    { id: 49, url: 'https://picsum.photos/400/300?random=49', title: 'Image 49' },
    { id: 50, url: 'https://picsum.photos/400/300?random=50', title: 'Image 50' },
  ];

  return (
    <Layout>
      <h2 className="text-3xl font-bold mb-6">Latest Images</h2>
      <ImageGrid images={images} />
    </Layout>
  );
};

export default App;
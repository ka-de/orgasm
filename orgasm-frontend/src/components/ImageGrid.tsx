import React from 'react';

interface Image {
  id: number;
  url: string;
  title: string;
}

interface ImageGridProps {
  images: Image[];
}

const ImageGrid: React.FC<ImageGridProps> = ({ images }) => {
  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
      {images.map((image) => (
        <div key={image.id} className="bg-card rounded-lg overflow-hidden shadow-md">
          <img src={image.url} alt={image.title} className="w-full h-48 object-cover" />
          <div className="p-4">
            <h3 className="text-lg font-semibold">{image.title}</h3>
          </div>
        </div>
      ))}
    </div>
  );
};

export default ImageGrid;
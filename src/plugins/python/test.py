from PIL import Image
from numpy import asarray
import lenna_core
print(lenna_core.resize.description())

image = Image.open('lenna.png')
data = asarray(image)
print(data.shape)

config = lenna_core.resize.default_config()
config['width'] = 200

print(config)
processed = lenna_core.resize.process(config, data)
print(processed.shape)
Image.fromarray(processed).save('lenna_test_out.png')

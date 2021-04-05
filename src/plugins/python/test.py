from PIL import Image
from numpy import asarray
import lenna_core_py
print(lenna_core_py.Resize.description())

image = Image.open('lenna.png')
data = asarray(image)
print(data.shape)

config = lenna_core_py.Resize.default_config()
config['width'] = 200

print(config)
processed = lenna_core_py.Resize.process(config, data)
print(processed.shape)
Image.fromarray(processed).save('lenna_test_out.png')

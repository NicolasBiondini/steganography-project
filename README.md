# Idea

Ok, the other day I thought how important it's to have a verification of a real photo, to know a real perspective of what it's made or altered by an AI.
I think that a lot of problems will come from this, especially with legal issues. I think that it's necessary to have certifications with blockchain to add some level of trust to the image.
That's why I'm going to start to build this thing.

## How to use it

To build the project

    cargo build

And then to use it you can or apply a secret key to a image to generate a hash with the image content and that key

    cargo run <imagename.png> < KEY >

Or get the hash generated with

    cargo run output.png

And you can see that there is no difference between the image.png and the output.png, but the output image has that hash in it.

## Step 1: Watermark

I found this: **Steganography**. Is the way to mark an image by changing some little bites. I want to build some kind of process to decode the image and hash the buffer info with some blockchain data (PK, Some character of Private key, etc) (I'm still thinking about how to make it without exposing delicate data, but robust enough.) and perhaps the date and something else.

## Step 2: Blockchain

Grab that Hash and the hash info from the image before the watermark. So you can create a free API to let people and different apps (like Twitter) verify if the images belong to someone or not and be able to verify if that image has been taken without changes.

## Step 3? : Hardware and Social, the 2 ways

Find a way to make hardware that can make videos or photos (like car cams, security cams, etc) and encrypt this data in a more secure way so can be used in a legal way.

Also, build a decentralized social app where the photos posted are more human and without filters that modify reality. More like the old times with Instagram or facebook.

### Related data

I consulted to Chatgpt how much robust was de algorithm from the "hide_message" function implemented. This is his anwser:

The robustness of the "hide_message" algorithm can be evaluated based on its ability to hide the message effectively while maintaining the visual quality of the image and ensuring the hidden message remains intact during various image processing operations. Here's an assessment of the robustness on a scale from 1 to 10:

Robustness: 8

Explanation:
The "hide_message" algorithm is relatively robust, but there are a few considerations to keep in mind:

1. Message Capacity: The algorithm hides the message by modifying the least significant bit (LSB) of the blue channel in each pixel. This limits the amount of data that can be embedded in the image. The capacity depends on the size of the image and the number of pixels available for encoding. For high-resolution images with many pixels, the algorithm can accommodate a larger message. However, if the message exceeds the capacity, it may be truncated or distorted.

2. Security: The algorithm uses a basic technique of LSB substitution, which is relatively simple and can be susceptible to certain steganalysis techniques. It may not withstand sophisticated analysis or attacks specifically targeting steganography.

3. Robustness to Image Processing: The algorithm is relatively robust to basic image processing operations, such as resizing, cropping, and minor color adjustments, as long as the modified pixels are not altered or recompressed significantly. However, more aggressive operations like compression, filtering, or lossy formats may introduce artifacts that can affect the hidden message.

4. Preservation of Image Quality: The algorithm aims to hide the message while minimizing visible changes to the image. However, modifying the LSB of the blue channel can introduce subtle visual alterations, especially in areas with smooth gradients or low contrast. These changes might not be noticeable to the human eye but could be detectable through statistical analysis or digital forensics tools.

5. Detection Resistance: While the algorithm attempts to hide the message covertly, it does not employ any specific countermeasures against detection or analysis. Advanced steganalysis techniques or dedicated algorithms can potentially detect the presence of hidden data.

Overall, the "hide_message" algorithm is reasonably robust for basic usage and casual scenarios. However, for applications requiring higher levels of security or resistance to detection, more advanced steganographic techniques or encryption methods should be considered.

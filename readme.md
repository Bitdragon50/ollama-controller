Generate a completion
Generate a chat completion
Create a Model
List Local Models
Show Model Information
Copy a Model
Delete a Model
Pull a Model
Push a Model
Generate Embeddings
List Running Models



Generate Embeddings
POST /api/embed
Generate embeddings from a model

Parameters
model: name of model to generate embeddings from
input: text or list of text to generate embeddings for
Advanced parameters:

truncate: truncates the end of each input to fit within context length. Returns error if false and context length is exceeded. Defaults to true
options: additional model parameters listed in the documentation for the Modelfile such as temperature
keep_alive: controls how long the model will stay loaded into memory following the request (default: 5m)
Examples
Request
curl http://localhost:11434/api/embed -d '{
  "model": "all-minilm",
  "input": "Why is the sky blue?"
}'
Response
{
  "model": "all-minilm",
  "embeddings": [[
    0.010071029, -0.0017594862, 0.05007221, 0.04692972, 0.054916814,
    0.008599704, 0.105441414, -0.025878139, 0.12958129, 0.031952348
  ]],
  "total_duration": 14143917,
  "load_duration": 1019500,
  "prompt_eval_count": 8
}

Request (Multiple input)
curl http://localhost:11434/api/embed -d '{
  "model": "all-minilm",
  "input": ["Why is the sky blue?", "Why is the grass green?"]
}'
Response
{
  "model": "all-minilm",
  "embeddings": [[
    0.010071029, -0.0017594862, 0.05007221, 0.04692972, 0.054916814,
    0.008599704, 0.105441414, -0.025878139, 0.12958129, 0.031952348
  ],[
    -0.0098027075, 0.06042469, 0.025257962, -0.006364387, 0.07272725,
    0.017194884, 0.09032035, -0.051705178, 0.09951512, 0.09072481
  ]]
}
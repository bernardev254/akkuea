// app/api/chat/route.ts

import { openai } from '@ai-sdk/openai';
import { streamText } from 'ai';
import { convertToCoreMessages } from 'ai';

// Ensure this is running as an Edge Function
export const runtime = 'edge';

// Allow streaming responses up to 30 seconds
export const maxDuration = 30;

export async function POST(req: Request) {
  try {
    const { messages } = await req.json();

    const result = streamText({
      model: openai('gpt-4o'), // Consider using 'gpt-4o' for better performance
      messages: convertToCoreMessages(messages),
      system: "You are a helpful AI assistant. Provide concise, accurate, and friendly responses to the user's questions.",
      onError({ error }) {
        console.error('AI Stream Error:', error);
      },
    });

    return result.toDataStreamResponse({
      headers: {
        'Transfer-Encoding': 'chunked',
        Connection: 'keep-alive',
      },
    });
  } catch (error) {
    console.error('API Route Error:', error);
    return new Response('Internal Server Error', { status: 500 });
  }
}

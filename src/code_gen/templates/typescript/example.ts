export async function myFunc(
  body: BodyInit,
  myVariable: string
): Promise<string> {
  const response = await fetch('https://someurl.com/etc', {
    headers: {
      'user-agent': 'blahblah',
      'content-type': 'application/json',
      'x-some-header': myVariable,
    },
    body,
    method: 'GET',
  })

  return await response.text()
}

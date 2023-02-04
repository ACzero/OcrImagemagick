defmodule OcrImagemagick do
  use Rustler, otp_app: :ocr_imagemagick, crate: "ocr_imagemagick"

  @spec jpg_combine(list(binary()))
  def jpg_combine(jpg_list), do: error()

  @spec jpg_combine(list(binary()))
  def jpg_combine_reverse(jpg_list), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
